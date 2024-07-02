# `SPIT` clause

As SurrealDB supports arrays and nested fields within arrays, it is possible
to split the result on a specific field name, returning each value in an array
as a separate value, along with the record content itself. This is useful in
data analysis contexts.

## Usage

```sql
-- Split the results by each value in an array
SELECT * FROM user SPLIT emails;

-- Split the results by each value in a nested array
SELECT * FROM country SPLIT locations.cities;

-- Filter the result of a subquery
SELECT * FROM (SELECT * FROM person SPLIT loggedin) WHERE loggedin > '2023-05-01';
```
