import os
import time
import json
import torch
import shutil
import pickle
import torch.optim as optim
from functools import partial
from torch_geometric.data import Data as PyGData
from torch_geometric.nn import ComplEx, DistMult, RotatE, TransE
from typing import Dict, Any, List, Tuple
from torch_geometric.transforms import RandomLinkSplit

# More details on how to train a kge model can be found at https://github.com/pyg-team/pytorch_geometric/blob/master/examples/kge_fb15k_237.py


def ensure_device(kwargs: Dict[str, Any]):
    device = kwargs.get("device")
    device_map = kwargs.get("device_map")
    if device is None and device_map is None:
        if torch.cuda.is_available():
            kwargs["device"] = "cuda:" + str(os.getpid() % torch.cuda.device_count())
        else:
            kwargs["device"] = "cpu"


def remove_unused_parameters(hyperparams: Dict[str, Any]) -> Dict[str, Any]:
    # Delete the hyperparams that are not used by the model
    c_hyperparams = hyperparams.copy()
    for key in [
        "batch_size",
        "shuffle",
        "learning_rate",
        "epochs",
        "save_path",
        "embedding_dim",
        "device_map",
        "device",
    ]:
        if key in c_hyperparams:
            del c_hyperparams[key]
    return c_hyperparams


ModelClassMap = {
    "TransE": TransE,
    "ComplEx": ComplEx,
    "DistMult": DistMult,
    "RotatE": RotatE,
}


class KGEModel:
    def __init__(
        self,
        model_name: str,
        # Tuple[head, relation, tail]
        train_data: PyGData,
        test_data: PyGData,
        index_map: Dict[str, Any] = None,
        **kwargs,
    ):
        """TransE model for Knowledge Graph Embedding.

        Args:
            train_data (PyGData): a pytorch geometric data object for training.
            test_data (PyGData): a pytorch geometric data object for testing.
            **kwargs: other hyperparameters for the model. such as batch_size, shuffle, learning_rate, device, embedding_dim, device_map, epochs, save_path, etc. (see https://pytorch-geometric.readthedocs.io/en/latest/modules/nn.html#torch_geometric.nn.models.TransE). If you add new hyperparameters, please also modify the remove_unused_parameters function to remove the unused hyperparameters for the model.
        """
        self.hyperparams = kwargs

        ensure_device(self.hyperparams)
        device = self.hyperparams["device"]
        batch_size = self.hyperparams.get("batch_size", 1000)
        shuffle = self.hyperparams.get("shuffle", True)
        learning_rate = self.hyperparams.get("learning_rate", 0.01)
        embedding_dim = self.hyperparams.get("embedding_dim", 400)

        self.train_data = train_data.to(device)
        self.test_data = test_data.to(device)

        num_nodes = self.train_data.num_nodes
        num_edge_types = self.train_data.num_edge_types

        self.ModelClass = ModelClassMap.get(model_name)
        if self.ModelClass is None:
            raise ValueError(f"model_name must be one of {ModelClassMap.keys()}")

        print("Run training on", device)
        self.model = self.ModelClass(
            num_nodes=num_nodes,
            num_relations=num_edge_types,
            hidden_channels=embedding_dim,
            **remove_unused_parameters(self.hyperparams),
        ).to(device)

        # For saving the model
        self.metadata = {
            "num_nodes": num_nodes,
            "num_edge_types": num_edge_types,
            "model_name": model_name,
            "hyperparams": self.hyperparams,
        }

        if index_map is not None:
            self.index_map = index_map

        self.loader = self.model.loader(
            head_index=self._get_head_index(self.train_data),
            rel_type=self._get_rel_type(self.train_data),
            tail_index=self._get_tail_index(self.train_data),
            batch_size=batch_size,
            shuffle=shuffle,
        )

        self.optimizer = optim.Adam(self.model.parameters(), lr=learning_rate)

    def get_train_data(self):
        """Returns the training data.

        Returns:
            PyGData: the training data.
        """
        return self.train_data
    
    def get_test_data(self):
        """Returns the test data.

        Returns:
            PyGData: the test data.
        """
        return self.test_data

    def get_model(self):
        """Returns the model.

        Returns:
            KGEModel: the model.
        """
        return self.model
    
    def get_loader(self):
        """Returns the loader.

        Returns:
            DataLoader: the loader.
        """
        return self.loader
    
    def get_optimizer(self):
        """Returns the optimizer.

        Returns:
            Optimizer: the optimizer.
        """
        return self.optimizer

    def get_metadata(self) -> Dict[str, Any]:
        """Returns the metadata of the model.

        Returns:
            Dict[str, Any]: a dictionary of metadata.
        """
        return self.metadata

    def get_metrics(self) -> Dict[str, float]:
        """Returns the metrics of the model.

        Returns:
            Dict[str, float]: a dictionary of metrics, including mr1, mr5, mr10, hit1, hit5, hit10, score_time, train_time.
        """
        return self.metrics

    def get_index_map(self) -> Dict[str, Any]:
        """Returns the index map for converting the labels to indices and vice versa.

        Returns:
            Dict[str, Any]: a dictionary of index maps.
        """
        return self.index_map

    def _get_head_index(self, data: PyGData) -> torch.Tensor:
        return data.edge_index[0]

    def _get_tail_index(self, data: PyGData) -> torch.Tensor:
        return data.edge_index[1]

    def _get_rel_type(self, data: PyGData) -> torch.Tensor:
        return data.edge_type

    def train_one_epoch(self) -> float:
        self.model.train()
        total_loss = total_examples = 0
        for head_index, rel_type, tail_index in self.loader:
            self.optimizer.zero_grad()
            loss = self.model.loss(head_index, rel_type, tail_index)
            loss.backward()
            self.optimizer.step()
            total_loss += float(loss) * head_index.numel()
            total_examples += head_index.numel()
        loss = total_loss / total_examples
        return loss

    def train(self) -> Dict[str, float]:
        """Trains the model.

        Raises:
            ValueError: if save_path is not provided.

        Returns:
            Dict[str, float]: a dictionary of metrics, including mr1, mr5, mr10, hit1, hit5, hit10, score_time, train_time.
        """
        epochs = self.hyperparams.get("epochs", 1000)
        batch_size = self.hyperparams.get("batch_size", 1000)
        save_path = self.hyperparams.get("save_path")

        if save_path is None:
            raise ValueError("save_path is required for training.")

        start = time.perf_counter()
        loss = None
        for epoch in range(1, epochs + 1):
            loss = self.train_one_epoch()
            print(f"Epoch: {epoch:03d}, Loss: {loss:.4f}")

        fit_time = time.perf_counter() - start
        if torch.cuda.is_available():
            torch.cuda.empty_cache()

        start = time.perf_counter()
        mr1, hit1 = self.test(k=1, batch_size=batch_size)
        mr5, hit5 = self.test(k=5, batch_size=batch_size)
        mr10, hit10 = self.test(k=10, batch_size=batch_size)

        metrics = {
            "mr1": mr1,
            "mr5": mr5,
            "mr10": mr10,
            "hit1": hit1,
            "hit5": hit5,
            "hit10": hit10,
            "loss": loss,
        }
        metrics["score_time"] = time.perf_counter() - start
        metrics["train_time"] = fit_time
        self.metrics = metrics

        print("Metrics:", metrics)

        if save_path is not None:
            # Save the results
            if os.path.isdir(save_path):
                shutil.rmtree(save_path, ignore_errors=True)

            os.makedirs(save_path)
            self.save_model(os.path.join(save_path, "model.pt"))

        return metrics

    @torch.no_grad()
    def test(self, k: int = 10, batch_size: int = 1000) -> Tuple[float, float]:
        """Evaluates the model quality by computing Mean Rank and Hits@k across all possible tail entities.

        Args:
            k (int, optional): The k in Hits@k. (default: 10)

        Returns:
            Tuple[float, float]: The Mean Rank and Hits@k across all possible tail entities.
        """
        self.model.eval()
        device = self.hyperparams.get("device")
        head_index = self._get_head_index(self.test_data)
        tail_index = self._get_tail_index(self.test_data)
        rel_type = self._get_rel_type(self.test_data)

        print("Run test on", device)
        return self.model.test(
            head_index=head_index,
            rel_type=rel_type,
            tail_index=tail_index,
            batch_size=batch_size,
            k=k,
        )

    def save_model(self, path: str):
        torch.save(self.model.state_dict(), path)

        with open(path + ".meta.json", "w") as f:
            json.dump(self.metadata, f)


def train(
    model_name: str,
    train_dataset: PyGData,
    test_dataset: PyGData,
    hyperparams: Dict[str, Any],
    index_map: Dict[str, Any],
) -> KGEModel:
    """Trains a KGE model.

    Args:
        model_name (str): a KGE model name. (TransE, ComplEx, DistMult, RotatE etc.)
        train_dataset (PyGData): a pytorch geometric data object for training. NOTE: the values must be integers.
        test_dataset (PyGData): a pytorch geometric data object for testing. NOTE: the values must be integers.
        hyperparams (Dict[str, Any]): a dictionary of hyperparameters. (see train function for details).
        index_map (Dict[str, Any]): a dictionary of index maps for converting the labels to indices and vice versa.

    Returns:
        KGEModel: a trained KGE model.
    """
    model = KGEModel(model_name, train_dataset, test_dataset, index_map, **hyperparams)
    model.train()
    return model


def train_with_hrt(
    model_name: str,
    hyperparams: Dict[str, Any],
    edges: Tuple[List[str], List[str], List[str]],
) -> Tuple[KGEModel, Dict[str, float]]:
    """Trains a KGE model with a tuple of (head, relation, tail) lists.

    Args:
        model_name (str): a KGE model name. (TransE, ComplEx, DistMult, RotatE etc.)
        edges (Tuple[List[str], List[str], List[str]]): a tuple of (head, relation, tail) lists.
        hyperparams (Dict[str, Any]): a dictionary of hyperparameters. (see train function for details). In addition, it also contains val_ratio, test_ratio, is_undirected parameters for splitting the dataset.

    Returns:
        Dict[str, float]: a dictionary of metrics, including mr1, mr5, mr10, hit1, hit5, hit10, score_time, train_time.
    """
    dataset, index_map = load_data(edges)

    val_ratio = hyperparams.get("val_ratio", 0.1)
    test_ratio = hyperparams.get("test_ratio", 0.2)
    is_undirected = hyperparams.get("is_undirected", False)
    train_dataset, validation_dataset, _ = split_dataset(
        dataset, val_ratio=val_ratio, test_ratio=test_ratio, is_undirected=is_undirected
    )

    cleaned_hyperparams = hyperparams.copy()
    for key in ["val_ratio", "test_ratio", "is_undirected"]:
        if key in cleaned_hyperparams:
            del cleaned_hyperparams[key]

    return train(model_name, train_dataset, validation_dataset, hyperparams, index_map)


def create_trainer(
    model_name: str,
    hyperparams: str,
):
    """Returns the correct trainer based on algorithm names we defined
    internally (see dict above).

    Parameters:
        - model_name: a KGE model name. (TransE, ComplEx, DistMult, RotatE etc.)
        - edges: a tuple of (head, relation, tail) lists.
        - hyperparams: JSON string of hyperparameters.
    """
    if hyperparams is None:
        hyperparams = {}
    else:
        hyperparams = json.loads(hyperparams)

    return partial(train_with_hrt, model_name, hyperparams)


def create_predictor(trainer):
    """Return the instantiated trainer.

    Parameters:
        - trainer: a trained trainer object.
    """

    def predict(h, r, t):
        """Predict the score of a given triple."""
        return predict_with_idx(trainer, h, r, t)

    return predict


def predict_with_idx(
    trainer,
    h: int,
    r: int,
    t: int,
) -> float:
    """Predicts the score of a given triple.

    Args:
        model_name (str): a KGE model name. (TransE, ComplEx, DistMult, RotatE etc.)
        model_path (str): the path of the model.
        h (int): a head entity.
        r (int): a relation.
        t (int): a tail entity.

    Returns:
        float: the score of the triple.
    """
    trainer.model.eval()
    with torch.no_grad():
        return trainer.model.forward(h, r, t)


def predict(
    trainer,
    h_label: str,
    r_label: str,
    t_label: str,
) -> float:
    """Predicts the score of a given triple.

    Args:
        trainer: an trainer object which contains all context information for prediction.
        h_label (str): a head entity which is a string. Such as Gene::ENTREZ:1017
        r_label (str): a relation which is a string. Such as GNBR::INTERACTS_WITH:Gene:Gene
        t_label (str): a tail entity which is a string. Such as Gene::ENTREZ:1017
        index_map_fpath (str): the path of the index map file for converting the labels to indices and vice versa.

    Returns:
        float: the score of the triple.
    """
    index_map = trainer.get_index_map()
    h = index_map["node_idx_dict"][h_label]
    r = index_map["relation_type_idx_dict"][r_label]
    t = index_map["node_idx_dict"][t_label]

    return predict_with_idx(trainer, h, r, t)


def pygdata2hrt(data: PyGData) -> Tuple[List[int], List[int], List[int]]:
    """Converts a PyGData object to a tuple of (head, relation, tail) lists.

    Args:
        data (PyGData): a PyGData object.

    Returns:
        Tuple[List[int], List[int], List[int]]: a tuple of (head, relation, tail) lists.
    """
    return (
        data.edge_index[0].tolist(),
        data.edge_type.tolist(),
        data.edge_index[1].tolist(),
    )


def hrt2pygdata(data: Tuple[List[int], List[int], List[int]]) -> PyGData:
    """Loads the data into a PyGData object.

    Args:
        data (Tuple[List[int], List[int], List[int]]):
            a tuple of (head, relation, tail) lists.

    Returns:
        Data: a PyGData object.
    """

    heads, relations, tails = data

    # Find unique nodes
    unique_nodes = set(heads + tails)
    num_nodes = len(unique_nodes)

    # Create edge idxs and types
    edge_index = torch.tensor([heads, tails], dtype=torch.long)
    edge_type = torch.tensor(relations, dtype=torch.long)

    # Create PyG Data object
    data = PyGData(edge_index=edge_index, edge_type=edge_type, num_nodes=num_nodes)

    return data


def load_data(
    edges: Tuple[List[str], List[str], List[str]]
) -> Tuple[PyGData, Dict[str, Any]]:
    """Loads the data into a PyGData object.

    Args:
        edges (Tuple[List[str], List[str], List[str]]):
            a tuple of (head, relation, tail) lists.

    Returns:
        Data: a PyGData object.
    """
    heads, relations, tails = edges

    head_index = []
    tail_index = []
    relation_index = []

    # Find unique nodes
    unique_nodes = set(heads + tails)
    unique_relation_types = set(relations)
    node_idx_dict = {node: idx for idx, node in enumerate(unique_nodes)}
    idx_node_dict = {idx: node for idx, node in enumerate(unique_nodes)}
    relation_type_idx_dict = {
        relation_type: idx for idx, relation_type in enumerate(unique_relation_types)
    }
    idx_relation_type_dict = {
        idx: relation_type for idx, relation_type in enumerate(unique_relation_types)
    }

    index_map = {
        "node_idx_dict": node_idx_dict,
        "idx_node_dict": idx_node_dict,
        "relation_type_idx_dict": relation_type_idx_dict,
        "idx_relation_type_dict": idx_relation_type_dict,
    }

    for head, relation, tail in zip(heads, relations, tails):
        head_index.append(node_idx_dict[head])
        tail_index.append(node_idx_dict[tail])
        relation_index.append(relation_type_idx_dict[relation])

    num_nodes = len(unique_nodes)

    # Create edge idxs and types
    edge_index = torch.tensor([head_index, tail_index], dtype=torch.long)
    edge_type = torch.tensor(relation_index, dtype=torch.long)

    # Create PyG Data object
    data = PyGData(edge_index=edge_index, edge_type=edge_type, num_nodes=num_nodes)

    return data, index_map


def split_dataset(
    data: PyGData,
    val_ratio: float = 0.1,
    test_ratio: float = 0.2,
    is_undirected: bool = False,
) -> Tuple[PyGData, PyGData, PyGData]:
    """Splits the dataset into train, validation and test datasets.

    Args:
        data (PyGData): a PyGData object.
        val_ratio (float, optional): a ratio for the validation dataset. Defaults to 0.1.
        test_ratio (float, optional): a ratio for the test dataset. Defaults to 0.2.
        is_undirected (bool, optional): whether the graph is undirected. Defaults to False.

    Returns:
        Tuple[PyGData, PyGData, PyGData]: a tuple of (train, validation, test) datasets.
    """
    transform = RandomLinkSplit(
        is_undirected=is_undirected, num_test=test_ratio, num_val=val_ratio
    )
    train, validation, test = transform(data)
    return train, validation, test


def save(trainer):
    """Save the estimtator as bytes (pickle).

    Parameters:
        - trainer: KGE trainer, instantiated.

    Return:
        bytes
    """
    return pickle.dumps(trainer)

def save_to_file(trainer, path):
    """Save the estimtator as bytes (pickle).

    Parameters:
        - trainer: KGE trainer, instantiated.

    Return:
        bytes
    """
    with open(path, 'wb') as f:
        pickle.dump(trainer, f)


def load(data):
    """Load the trainer from bytes (pickle).

    Parameters:
        - data: bytes

    Return:
        KGE trainer
    """
    return pickle.loads(bytes(data))


if __name__ == "__main__":
    from torch_geometric.datasets import FB15k_237

    train_dataset = FB15k_237(root="examples/FB15k-237", split="train")[0]
    test_dataset = FB15k_237(root="examples/FB15k-237", split="test")[0]

    model_name = "TransE"

    hyperparams = {
        "epochs": 10,
        "batch_size": 1000,
        "learning_rate": 0.01,
        "device": None,
        "device_map": None,
        "save_path": os.path.join("examples", "FB15k-237", "TransE"),
    }

    metrics = train(
        model_name, train_dataset, test_dataset, hyperparams, hyperparams["save_path"]
    )
