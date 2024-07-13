# `WITH` clause

The `WITH` clause allows for manual control over query optimisations by
specifying whether to use index iterators or the standard table iterator.
This can be useful in situations where the automatic query planner might not
choose the most efficient path, especially when dealing with high cardinality
indexes.

## Usage

- **WITH INDEX @indexes**: Restricts the query planner to use only the specified index(es).
- **WITH NOINDEX**: Forces the query planner to use the table iterator.

```sql
-- forces the query planner to use the specified index(es):
SELECT * FROM person WITH INDEX ft_email WHERE email = 'tobie@surrealdb.com' AND company = 'SurrealDB';

-- forces the usage of the table iterator
SELECT name FROM person WITH NOINDEX WHERE job = 'engineer' AND gender = 'm';
```
