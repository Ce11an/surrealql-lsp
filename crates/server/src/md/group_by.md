# `GROUP BY` clause

SurrealDB supports data aggregation and grouping, with support for multiple
fields, nested fields, and aggregate functions. In SurrealDB, every field which
appears in the field projections of the select statement (and which is not an
aggregate function), must also be present in the GROUP BY clause.

## Usage

```sql
-- Group records by a single field
SELECT country FROM user GROUP BY country;

-- Group results by a nested field
SELECT settings.published FROM article GROUP BY settings.published;

-- Group results by multiple fields
SELECT gender, country, city FROM person GROUP BY gender, country, city;

-- Group results with aggregate functions
SELECT count() AS total, math::mean(age) AS average_age, gender, country FROM person GROUP BY gender, country;
```
