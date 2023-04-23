-- Pull the Primary keys from all tables
WITH primary_keys AS (
    SELECT 1
    FROM information_schema.table_constraints AS tc
    INNER JOIN information_schema.constraint_column_usage AS ccu
        ON
            tc.constraint_schema = ccu.constraint_schema
            AND tc.constraint_name = ccu.constraint_name
    WHERE tc.constraint_type = 'PRIMARY KEY'
),

-- Create a CTE which contains all tables and views which need to be mapped
tables_and_views AS (
    SELECT
        schemaname,
        tablename,
        'table' AS ty
    FROM pg_catalog.pg_tables
    WHERE schemaname != 'pg_catalog' AND schemaname != 'information_schema'

    UNION

    SELECT
        table_schema AS schemaname,
        table_name AS tablename,
        'view' AS ty
    FROM information_schema.views
    WHERE table_schema = ANY(CURRENT_SCHEMAS(false))

)

SELECT
    things.schemaname AS schema,
    things.tablename AS table_name,
    things.ty,
    col.column_name,
    col.udt_name AS column_type,
    CASE WHEN col.is_nullable = 'YES' THEN 1 ELSE 0 END AS is_nullable,
    CASE WHEN (
        SELECT 1 FROM primary_keys AS keys
        WHERE
            keys.constraint_schema = things.schemaname
            AND keys.table_name = things.tablename
            AND keys.column_name = col.column_name
    ) IS NOT NULL THEN 1 ELSE 0 END AS is_primary_key,
    CASE WHEN col.is_updatable = 'YES' THEN 1 ELSE 0 END AS is_updatable
FROM tables_and_views AS things
INNER JOIN information_schema.columns AS col
    ON
        col.table_schema = things.schemaname
        AND col.table_name = things.tablename
ORDER BY
    things.schemaname ASC,
    things.tablename ASC,
    things.is_primary_key DESC,
    col.column_name ASC
