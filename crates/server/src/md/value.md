# `VALUE` clause

The `VALUE` keyword in SurrealDB is used to return specific fields as an array
of values instead of the default array of objects. This feature is particularly
useful when you want to retrieve a single un-nested field from a table or a
specific record.

## Usage

```sql
-- Store the subquery result in a variable and query that result.
LET $history = SELECT * FROM events WHERE type = 'activity' LIMIT 5;
SELECT * from $history;

-- Use the parent instance's field in a subquery (predefined variable)
SELECT *, (SELECT * FROM events WHERE host == $parent.id) AS hosted_events FROM user;
```

