# `ORDER BY` clause

Sort records using the `ORDER BY` clause.
To sort records, SurrealDB allows ordering on multiple fields and nested fields.
Use the `ORDER BY` clause to specify a comma-separated list of field names that
should be used to order the resulting records. The `ASC` and `DESC` keywords
can be used to specify whether results should be sorted in an ascending or
descending manner.

The `COLLATE` keyword can be used to use unicode collation when ordering text in
string values, ensuring that different cases, and different languages are sorted
in a consistent manner. Finally, the `NUMERIC` can be used to correctly sort text
which contains numeric values.

## Usage

```sql
-- Order records randomly
SELECT * FROM user ORDER BY RAND();

-- Order records descending by a single field
SELECT * FROM song ORDER BY rating DESC;

-- Order records by multiple fields independently
SELECT * FROM song ORDER BY artist ASC, rating DESC;

-- Order text fields with unicode collation
SELECT * FROM article ORDER BY title COLLATE ASC;

-- Order text fields with which include numeric values
SELECT * FROM article ORDER BY title NUMERIC ASC;
```
