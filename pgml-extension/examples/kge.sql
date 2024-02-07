-- Row by row KGE example, 4.11 seconds
SELECT
    ee1.entity_id AS head,
    rte.relation_type AS relation_type,
    ee2.entity_id AS tail,
	pgml.transe_l2_ndarray(
			vector_to_float4(ee1.embedding, 400, false),
			vector_to_float4(rte.embedding, 400, false),
			vector_to_float4(ee2.embedding, 400, false),
			12.0,
			true
	) AS score	
FROM
    biomedgps_entity_embedding ee1,
    biomedgps_relation_embedding rte,
    biomedgps_entity_embedding ee2
WHERE
    ee1.entity_id = 'ENTREZ:6747'
    AND rte.relation_type = 'STRING::BINDING::Gene:Gene'
GROUP BY
    ee1.embedding_id,
    rte.embedding_id,
    ee2.embedding_id
ORDER BY score DESC
LIMIT 10

-- Combined rows KGE example
SELECT
    ee1.entity_id AS head,
    rte.relation_type AS relation_type,
    ee2.entity_id AS tail,
	ee2.embedding AS embedding
FROM
    biomedgps_entity_embedding ee1,
    biomedgps_relation_embedding rte,
    biomedgps_entity_embedding ee2
WHERE
    ee1.entity_id = 'ENTREZ:6747' 
    AND rte.relation_type = 'STRING::BINDING::Gene:Gene'
	AND ee2.entity_type = 'Gene'
GROUP BY
    ee1.embedding_id,
    rte.embedding_id,
    ee2.embedding_id;

-- Parallel KGE example, 1 second
WITH tail_embeddings_agg AS (
    SELECT
	    ARRAY_AGG(entity_id) AS tails,
        ARRAY_AGG(vector_to_float4(embedding, 400, false)) AS tail_emb_agg
    FROM
        biomedgps_entity_embedding
),
selected_head_rel AS (
    SELECT
        ee1.entity_id AS head,
        rte.relation_type AS relation_type,
        vector_to_float4(ee1.embedding, 400, false) AS head_emb,
        vector_to_float4(rte.embedding, 400, false) AS rel_emb
    FROM
        biomedgps_entity_embedding ee1,
		biomedgps_relation_embedding rte
    WHERE
        ee1.entity_id = 'ENTREZ:6747'
        AND rte.relation_type = 'STRING::BINDING::Gene:Gene'
	GROUP BY
	    ee1.embedding_id,
	    rte.embedding_id
    LIMIT 1
)
SELECT
    shr.head,
    shr.relation_type,
	unnest(tea.tails) AS tail,
    unnest(pgml.transe_l2_parallel(
        shr.head_emb,
        shr.rel_emb,
        tea.tail_emb_agg,
        12.0,
        true
    )) AS score
FROM
    selected_head_rel shr,
    tail_embeddings_agg tea
ORDER BY score DESC
LIMIT 10
