-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_module_function_module_move_module_ref;
DROP TABLE IF EXISTS module_function CASCADE;

DROP INDEX IF EXISTS mm_addr_name_ver_index;
DROP INDEX IF EXISTS mm_insat_index;
DROP TABLE IF EXISTS move_modules;