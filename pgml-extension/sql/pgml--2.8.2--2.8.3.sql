-- src/kge.rs
-- pgml::kge::logsigmoid
DROP FUNCTION IF EXISTS pgml."logsigmoid"(real);
CREATE  FUNCTION pgml."logsigmoid"(
    "x" REAL /* f32 */
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'logsigmoid_wrapper';

-- src/kge.rs
-- pgml::kge::mean
DROP FUNCTION IF EXISTS pgml."mean"(real[]);
CREATE  FUNCTION pgml."mean"(
    "x" REAL[] /* f32[] */
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'mean_wrapper';

-- src/kge.rs
-- pgml::kge::median
DROP FUNCTION IF EXISTS pgml."median"(real[]);
CREATE  FUNCTION pgml."median"(
    "x" REAL[] /* f32[] */
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'median_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l2_parallel
DROP FUNCTION IF EXISTS pgml."transe_l2_parallel"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l2_parallel"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL[] /* f32[] */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l2_parallel_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l2_ndarray
DROP FUNCTION IF EXISTS pgml."transe_l2_ndarray"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l2_ndarray"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l2_ndarray_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l2
DROP FUNCTION IF EXISTS pgml."transe_l2"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l2"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l2_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l1
DROP FUNCTION IF EXISTS pgml."transe_l1"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l1"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l1_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l1_ndarray
DROP FUNCTION IF EXISTS pgml."transe_l1_ndarray"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l1_ndarray"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l1_ndarray_wrapper';

-- src/kge.rs
-- pgml::kge::transe_l1_parallel
DROP FUNCTION IF EXISTS pgml."transe_l1_parallel"(real[], real[], real[], real, bool, bool);
CREATE  FUNCTION pgml."transe_l1_parallel"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "gamma" REAL /* f32 */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL[] /* f32[] */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'transe_l1_parallel_wrapper';

-- src/kge.rs
-- pgml::kge::distmult
DROP FUNCTION IF EXISTS pgml."distmult"(real[], real[], real[], bool, bool);
CREATE  FUNCTION pgml."distmult"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'distmult_wrapper';

-- src/kge.rs
-- pgml::kge::distmult_ndarray
DROP FUNCTION IF EXISTS pgml."distmult_ndarray"(real[], real[], real[], bool, bool);
CREATE  FUNCTION pgml."distmult_ndarray"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'distmult_ndarray_wrapper';

-- src/kge.rs
-- pgml::kge::complex
DROP FUNCTION IF EXISTS pgml."complex"(real[], real[], real[], bool, bool);
CREATE  FUNCTION pgml."complex"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'complex_wrapper';

-- src/kge.rs
-- pgml::kge::complex_ndarray
DROP FUNCTION IF EXISTS pgml."complex_ndarray"(real[], real[], real[], bool, bool);
CREATE  FUNCTION pgml."complex_ndarray"(
    "head_vector" REAL[] /* f32[] */,
    "relation_vector" REAL[] /* f32[] */,
    "tail_vector" REAL[] /* f32[] */,
    "exp_enabled" BOOLEAN,
    "reverse" BOOLEAN
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'complex_ndarray_wrapper';
