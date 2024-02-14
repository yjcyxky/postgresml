-- src/kge.rs:11
-- pgml::kge::logsigmoid
DROP FUNCTION IF EXISTS pgml."logsigmoid"(real);
CREATE  FUNCTION pgml."logsigmoid"(
    "x" REAL /* f32 */
) RETURNS REAL /* f32 */
IMMUTABLE STRICT PARALLEL SAFE
LANGUAGE c /* Rust */
AS 'MODULE_PATHNAME', 'logsigmoid_wrapper';

-- src/kge.rs:24
-- pgml::kge::transe_l2_parallel
DROP FUNCTION IF EXISTS pgml."transe_l2_parallel"(real[], real[], real[], real, bool);
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

-- src/kge.rs:89
-- pgml::kge::transe_l2_ndarray
DROP FUNCTION IF EXISTS pgml."transe_l2_ndarray"(real[], real[], real[], real, bool);
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

-- src/kge.rs:127
-- pgml::kge::transe_l2
DROP FUNCTION IF EXISTS pgml."transe_l2"(real[], real[], real[], real, bool);
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

-- src/kge.rs:168
-- pgml::kge::transe_l1
DROP FUNCTION IF EXISTS pgml."transe_l1"(real[], real[], real[], real, bool);
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

-- src/kge.rs:207
-- pgml::kge::transe_l1_ndarray
DROP FUNCTION IF EXISTS pgml."transe_l1_ndarray"(real[], real[], real[], real, bool);
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

-- src/kge.rs:237
-- pgml::kge::transe_l1_parallel
DROP FUNCTION IF EXISTS pgml."transe_l1_parallel"(real[], real[], real[], real, bool);
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

-- src/kge.rs:299
-- pgml::kge::distmult
DROP FUNCTION IF EXISTS pgml."distmult"(real[], real[], real[], bool);
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

-- src/kge.rs:335
-- pgml::kge::distmult_ndarray
DROP FUNCTION IF EXISTS pgml."distmult_ndarray"(real[], real[], real[], bool);
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
