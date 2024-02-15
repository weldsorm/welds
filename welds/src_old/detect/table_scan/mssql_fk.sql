SELECT 
  schema_name(tb_src.schema_id) as schema_name,
  tb_src.name as table_name,
  col_src.name as column_name,
  schema_name(tb_dest.schema_id) as schema_name,
  tb_dest.name as table_name,
  col_dest.name as fk_column_name
FROM sys.foreign_keys fk
JOIN sys.foreign_key_columns fk_c on fk.object_id = fk_c.constraint_object_id
JOIN sys.columns col_src on fk_c.parent_object_id = col_src.object_id AND col_src.column_id = parent_column_id
JOIN sys.tables tb_src on tb_src.object_id = fk_c.parent_object_id
JOIN sys.columns col_dest on fk_c.referenced_object_id = col_dest.object_id AND col_dest.column_id = fk_c.referenced_column_id
JOIN sys.tables tb_dest on tb_dest.object_id = fk_c.referenced_object_id


