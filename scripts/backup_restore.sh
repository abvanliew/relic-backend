mongodump --db relic --out /data/backup/`date +"%Y-%m-%d"`
mongorestore --db relic-restore /data/backup/`date +"%Y-%m-%d"`/relic