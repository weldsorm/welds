SELECT 
    null as schemaname,
    m.name,
    p."from" as from_column,
    null as fk_schemaname,
    p."table" as to_table,
    p."to" as to_column
FROM
    sqlite_master m
    JOIN pragma_foreign_key_list(m.name) p ON m.name != p."table"
WHERE m.type = 'table'
ORDER BY m.name
