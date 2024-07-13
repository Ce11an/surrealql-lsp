# `TIMEOUT` clause

When processing a large result set with many interconnected records, it is
possible to use the TIMEOUT keyword to specify a timeout duration for the
statement. If the statement continues beyond this duration, then the transaction
will fail, and the statement will return an error.

## Usage

```sql
-- Cancel this conditional filtering based on graph edge properties
-- if it's not finished within 5 seconds
SELECT * FROM person WHERE ->knows->person->(knows WHERE influencer = true) TIMEOUT 5s;
```
