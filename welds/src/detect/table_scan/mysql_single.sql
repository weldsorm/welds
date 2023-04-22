SELECT 
tv.table_schema, 
tv.table_name, 
tv.ty, 
col.column_name, 
col.data_type, 
cast(col.is_nullable='YES' AS SIGNED INTEGER) as is_nullable,
case when column_key= 'PRI' then 1 else 0 end as is_primary_key,
1 as is_updatable
FROM (
select 
table_name, table_schema, 'table' as ty 
from information_schema.tables 
where TABLE_TYPE like 'BASE TABLE'
AND table_schema != 'sys' AND table_schema != 'performance_schema' AND table_schema != 'information_schema' 
AND ( concat(table_schema, '::', table_name) != 'mysql::columns_priv' )
AND ( concat(table_schema, '::', table_name) != 'mysql::component' )
AND ( concat(table_schema, '::', table_name) != 'mysql::db' )
AND ( concat(table_schema, '::', table_name) != 'mysql::default_roles' )
AND ( concat(table_schema, '::', table_name) != 'mysql::engine_cost' )
AND ( concat(table_schema, '::', table_name) != 'mysql::func' )
AND ( concat(table_schema, '::', table_name) != 'mysql::general_log' )
AND ( concat(table_schema, '::', table_name) != 'mysql::global_grants' )
AND ( concat(table_schema, '::', table_name) != 'mysql::gtid_executed' )
AND ( concat(table_schema, '::', table_name) != 'mysql::help_category' )
AND ( concat(table_schema, '::', table_name) != 'mysql::help_keyword' )
AND ( concat(table_schema, '::', table_name) != 'mysql::help_relation' )
AND ( concat(table_schema, '::', table_name) != 'mysql::help_topic' )
AND ( concat(table_schema, '::', table_name) != 'mysql::innodb_index_stats' )
AND ( concat(table_schema, '::', table_name) != 'mysql::innodb_table_stats' )
AND ( concat(table_schema, '::', table_name) != 'mysql::ndb_binlog_index' )
AND ( concat(table_schema, '::', table_name) != 'mysql::password_history' )
AND ( concat(table_schema, '::', table_name) != 'mysql::plugin' )
AND ( concat(table_schema, '::', table_name) != 'mysql::procs_priv' )
AND ( concat(table_schema, '::', table_name) != 'mysql::proxies_priv' )
AND ( concat(table_schema, '::', table_name) != 'mysql::replication_asynchronous_connection_failover' )
AND ( concat(table_schema, '::', table_name) != 'mysql::replication_asynchronous_connection_failover_managed' )
AND ( concat(table_schema, '::', table_name) != 'mysql::replication_group_configuration_version' )
AND ( concat(table_schema, '::', table_name) != 'mysql::replication_group_member_actions' )
AND ( concat(table_schema, '::', table_name) != 'mysql::role_edges' )
AND ( concat(table_schema, '::', table_name) != 'mysql::server_cost' )
AND ( concat(table_schema, '::', table_name) != 'mysql::servers' )
AND ( concat(table_schema, '::', table_name) != 'mysql::slave_master_info' )
AND ( concat(table_schema, '::', table_name) != 'mysql::slave_relay_log_info' )
AND ( concat(table_schema, '::', table_name) != 'mysql::slave_worker_info' )
AND ( concat(table_schema, '::', table_name) != 'mysql::slow_log' )
AND ( concat(table_schema, '::', table_name) != 'mysql::tables_priv' )
AND ( concat(table_schema, '::', table_name) != 'mysql::time_zone' )
AND ( concat(table_schema, '::', table_name) != 'mysql::time_zone_leap_second' )
AND ( concat(table_schema, '::', table_name) != 'mysql::time_zone_name' )
AND ( concat(table_schema, '::', table_name) != 'mysql::time_zone_transition' )
AND ( concat(table_schema, '::', table_name) != 'mysql::time_zone_transition_type' )
AND ( concat(table_schema, '::', table_name) != 'mysql::user' ) 
UNION
select table_name, table_schema, 'view' as ty  from information_schema.tables 
where TABLE_TYPE like 'VIEW'
AND table_schema != 'sys' AND table_schema != 'performance_schema' AND table_schema != 'information_schema' 
) tv
JOIN INFORMATION_SCHEMA.COLUMNS col on col.table_name = tv.table_name AND col.table_schema = col.table_schema

WHERE (tv.table_schema = ? OR (? is null AND tv.table_schema = DATABASE()) ) and tv.table_name = ?

ORDER BY tv.table_schema, tv.table_name, is_primary_key desc, col.column_name

