# `PARALLEL` clause

When processing a large result set with many interconnected records, it is
possible to use the `PARALLEL` keyword to specify that the statement should be
processed in parallel. This can significantly improve the performance of the
statement, but it is important to note that the statement will not be processed
in a transactional manner, and so the results may not be consistent.

## Usage

```sql
-- Fetch and process the person, purchased and product targets in parallel
-- Select every product that was purchased by a person that purchased a product that person tobie also purchased
SELECT ->purchased->product<-purchased<-person->purchased->product FROM person:tobie PARALLEL;
```
