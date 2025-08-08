# Distributed Locks

After reading ["Designing Data-Intensive Applications" by Martin Kleppmann](https://bookshop.org/a/116077/9781449373320). I decided to see how distributed locks work in practice.
In the past, I've used [Redis for distributed locks](https://redis.io/docs/latest/develop/clients/patterns/distributed-locks/) but I've long felt they could be better.


The main difference I see between Redlock's and other distributed locks (e.g. Zookeeper), is their use (or lack) of fencing tokens.
During my time at Noteable, we ran into many concurrency issues that could have been solved by this approach.


Specifically, I want to see how to use this approach across both a PostgreSQL database and S3.
