
select 
    things.schemaname as schema,
    things.tablename as table_name,
    things.ty,
    col.column_name,
    col.udt_name as column_type,
    case when col.is_nullable = 'YES' then 1 else 0 end as is_nullable,
    case when pks.column_name is not null then 1 else 0 end as is_primary_key,
    case when col.is_updatable = 'YES' then 1 else 0 end as is_updatable
FROM (
    SELECT schemaname, tablename, 'table' as ty FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'
    UNION
    SELECT table_schema as schemaname, table_name as tablename, 'view' as ty from INFORMATION_SCHEMA.views WHERE table_schema = ANY (current_schemas(false))
) things
join information_schema.columns col on col.table_schema = things.schemaname AND col.table_name = things.tablename
LEFT JOIN (
SELECT ccu.column_name, tc.constraint_schema, tc.table_name
FROM information_schema.table_constraints tc 
JOIN information_schema.constraint_column_usage AS ccu USING (constraint_schema, constraint_name)
WHERE tc.constraint_type='PRIMARY KEY'
) pks on pks.constraint_schema = things.schemaname AND pks.table_name = things.tablename AND pks.column_name = col.column_name
ORDER BY things.schemaname, things.tablename, is_primary_key desc, col.column_name
