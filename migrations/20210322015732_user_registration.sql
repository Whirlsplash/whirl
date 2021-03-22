CREATE TABLE "user_registration" (
	"user_name_lower" TEXT NOT NULL,
	"user_name" TEXT NOT NULL,
	"serial_number" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"client_version" TEXT NOT NULL,
	"account_status" INTEGER NOT NULL,
	"registration_date" TEXT NOT NULL,
	"times_on" INTEGER NOT NULL,
	"total_minutes" INTEGER NOT NULL,
	"user_privileges" INTEGER NOT NULL
);
