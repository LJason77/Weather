# Weather

[![build badge](https://github.com/LJason77/Weather/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/Weather/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/Weather?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/Weather?style=social)

## 部署

### 数据库

```shell
docker run -d --name mongo --restart always -e MONGO_INITDB_ROOT_USERNAME=mongo -e MONGO_INITDB_ROOT_PASSWORD=mongo -v $(pw)/mongo-init.js:/docker-entrypoint-initdb.d/mongo-init.js -v ~/.db/mongo:/data/db -p 27017:27017 mongo --wiredTigerCollectionBlockCompressor zlib
```

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/Weather)
