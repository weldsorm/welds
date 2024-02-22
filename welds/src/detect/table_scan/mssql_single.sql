SELECT 
    tv.table_schema,
    tv.table_name,
    tv.ty,
    c.name column_name,
    t.Name column_type,
    CAST(c.is_nullable as INT) as is_nullable,
    CAST(ISNULL(i.is_primary_key, 0) as INT) as is_primary_key,
    CAST(1 as INT) as is_updatable
FROM    
    sys.columns c
INNER JOIN 
    sys.types t ON c.user_type_id = t.user_type_id
LEFT OUTER JOIN 
    sys.index_columns ic ON ic.object_id = c.object_id AND ic.column_id = c.column_id
LEFT OUTER JOIN 
    sys.indexes i ON ic.object_id = i.object_id AND ic.index_id = i.index_id
JOIN (
SELECT table_schema, table_name, 'table' as ty FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME NOT in (select name from sys.objects where type = 'U' and is_ms_shipped = 1) AND TABLE_TYPE='BASE TABLE'
UNION
SELECT table_schema, table_name, 'view' as ty FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME NOT in (select name from sys.objects where type = 'V' and is_ms_shipped = 1) AND TABLE_TYPE='VIEW' 
) tv on c.object_id = OBJECT_ID(CONCAT(tv.table_schema, '.', tv.table_name) )

WHERE tv.table_schema = @p1 and tv.table_name = @p2

ORDER BY tv.table_schema, tv.table_name, is_primary_key desc, column_name
