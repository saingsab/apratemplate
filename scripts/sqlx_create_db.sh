#!/usr/bin/env bash
export DATABASE_URL=postgres://AKIAV2YJRM5FKNODTN:B2bLth6rTB4awGhulMiGabiu4CdWrBQ77QX50K@localhost:5432/db_test_mig
sqlx database create

#CREATE TPLE SCRIP
#touch migrations/${timestamp}_create_subscriptions_table.sql