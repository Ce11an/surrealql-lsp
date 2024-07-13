# `FROM` clause

Each `SELECT` statement supports selecting from multiple targets using the
`FROM` statement.

## Usage

```sql
-- Select all fields from a table
SELECT * FROM person;

-- Select specific fields from a specific record
SELECT name, address, email FROM person:tobie;
```
