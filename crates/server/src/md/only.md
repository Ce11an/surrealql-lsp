# `ONLY` clause

If you are selecting just one single resource, it's possible to use the `ONLY`
clause to filter that result from an array.

## Usage

```sql
SELECT * FROM ONLY person:john;
```

If you are selecting from a resource where it is possible that multiple
resources are returned, it is required to `LIMIT` the result to just one. This is
needed, because the query would otherwise not be deterministic.

```sql
-- Fails
SELECT * FROM ONLY table_name;

-- Succeeds
SELECT * FROM ONLY table_name LIMIT 1;
```

