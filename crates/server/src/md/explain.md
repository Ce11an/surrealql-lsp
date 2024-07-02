# `EXPLAIN` clause

When `EXPLAIN` is used, the SELECT statement returns an explanation,
essentially revealing the execution plan to provide transparency and
understanding of the query performance.

## Usage

```sql
-- Returns the execution plan
SELECT * FROM person WHERE email='tobie@surrealdb.com' EXPLAIN;

-- Here is the result when the field 'email' is not indexed. We can see that the execution plan will iterate over the whole table.
```

```json
[
  {
    detail: {
      table: 'person'
    },
    operation: 'Iterate Table'
  }
]
```

```sql
-- Returns the execution plan with the number of fetched rows
SELECT * FROM person WHERE email='tobie@surrealdb.com' EXPLAIN FULL;

-- Here is the result when the 'email' field is indexed. We can see that the execution plan will proceed by utilizing the index.
```

```json
[
  {
    detail: {
      plan: {
        index: 'email',
        operator: '=',
        value: 'tobie@surrealdb.com'
      },
      table: 'person'
    },
    operation: 'Iterate Index'
  },
  {
    detail: {
      count: 1
    },
    operation: 'Fetch'
  }
]
```

