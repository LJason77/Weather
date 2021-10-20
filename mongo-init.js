print('Start #################################################################');

db = db.getSiblingDB('weather');
db.createUser(
    {
        user: 'weather',
        pwd: 'weather',
        roles: [{ role: 'dbOwner', db: 'weather' }],
    },
);

// weather
db.weather.createIndex({"device": 1,"ts":1}, {background: true});

print('END #################################################################');
