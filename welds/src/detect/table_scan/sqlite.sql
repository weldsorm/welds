select
  null as schemaname,
  sqlite_master.name as tablename,
  ss.type as ty,
  table_info.name as column_name,
  table_info.type as column_type,
  NOT table_info."notnull" as is_nullable,
  table_info.pk as is_primary_key,
  1 as is_updatable
from
  sqlite_master
  join pragma_table_info(sqlite_master.name) as table_info
  JOIN sqlite_schema ss on ss.name = sqlite_master.name
order by
  sqlite_master.name, is_primary_key, column_name
