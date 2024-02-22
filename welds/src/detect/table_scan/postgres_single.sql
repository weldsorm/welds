
SELECT 
    things.schemaname as schema,
    things.tablename as table_name,
    things.ty,
    col.column_name,
    col.udt_name as column_type,
    case when col.is_nullable = 'YES' then 1 else 0 end as is_nullable,
    case when (
SELECT 1  
FROM information_schema.table_constraints AS tc
INNER JOIN
    information_schema.constraint_column_usage AS ccu
    ON
        tc.constraint_schema = ccu.constraint_schema
        AND tc.constraint_name = ccu.constraint_name
WHERE 
    tc.constraint_type = 'PRIMARY KEY'
    AND tc.constraint_schema = things.schemaname
    AND tc.table_name = things.tablename
    AND ccu.column_name = col.column_name

) is not null then 1 else 0 end as is_primary_key,
    case when col.is_updatable = 'YES' then 1 else 0 end as is_updatable
FROM (
    SELECT schemaname, tablename, 'table' as ty FROM pg_catalog.pg_tables WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'
    UNION
    SELECT table_schema as schemaname, table_name as tablename, 'view' as ty from INFORMATION_SCHEMA.views WHERE table_schema = ANY (current_schemas(false))
) things
join information_schema.columns col on col.table_schema = things.schemaname AND col.table_name = things.tablename
WHERE things.schemaname ilike $1 and things.tablename ilike $2
ORDER BY things.schemaname, things.tablename, is_primary_key desc, col.column_name




