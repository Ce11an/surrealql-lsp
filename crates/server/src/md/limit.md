# `LIMIT` clause

To limit the number of records returned, use the `LIMIT` clause.

## Usage

```sql
-- Select only the top 50 records from the person table
SELECT * FROM person LIMIT 50;
```

When using the LIMIT clause, it is possible to paginate results by using the
`START` clause to start from a specific record from the result set.

```sql
-- Start at record 50 and select the following 50 records
SELECT * FROM user LIMIT 50 START 50;
```
