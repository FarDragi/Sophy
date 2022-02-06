-- CreateTable
CREATE TABLE "user" (
    "id" UUID NOT NULL,
    "discord_id" CHAR(20) NOT NULL,
    "name" VARCHAR(100) NOT NULL,

    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "user_discord_id_key" ON "user"("discord_id");
