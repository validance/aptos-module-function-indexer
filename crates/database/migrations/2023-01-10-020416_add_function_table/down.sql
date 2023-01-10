-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_module_function_module_move_module_ref;
DROP TABLE IF EXISTS module_function CASCADE;