set shell := ["cmd.exe", "/C"]

generateEntities:
    sea-orm-cli generate entity -o entity/src

generateMigrations:
    sea-orm-cli migrate

generateAll:
    sea-orm-cli migrate
    sea-orm-cli generate entity -o entity/src