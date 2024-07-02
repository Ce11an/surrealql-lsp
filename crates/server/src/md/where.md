# `WHERE` clause

As with traditional SQL queries, the SurrealDB `SELECT` queries support
conditional filtering using a `WHERE` clause. If the expression in the `WHERE`
clause evaluates to true, then the respective record will be returned.

## Usage

```sql
-- Simple conditional filtering
SELECT * FROM article WHERE published = true;

-- Conditional filtering based on graph edges
SELECT * FROM profile WHERE count(->experience->organisation) > 3;

-- Conditional filtering based on graph edge properties
SELECT * FROM person WHERE ->(reaction WHERE type='celebrate')->post;

-- Conditional filtering with boolean logic
SELECT * FROM user WHERE (admin AND active) OR owner = true;

-- Select filtered nested array values
SELECT address[WHERE active = true] FROM person;
```

