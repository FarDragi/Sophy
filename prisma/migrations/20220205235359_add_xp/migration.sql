/*
  Warnings:

  - The primary key for the `user` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `discord_id` on the `user` table. All the data in the column will be lost.
  - Changed the type of `id` on the `user` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- DropIndex
DROP INDEX "user_discord_id_key";

-- AlterTable
ALTER TABLE "user" DROP CONSTRAINT "user_pkey",
DROP COLUMN "discord_id",
DROP COLUMN "id",
ADD COLUMN     "id" CHAR(20) NOT NULL,
ADD CONSTRAINT "user_pkey" PRIMARY KEY ("id");

-- CreateTable
CREATE TABLE "xp" (
    "id" CHAR(20) NOT NULL,
    "progress" BIGINT NOT NULL DEFAULT 0,
    "level" INTEGER NOT NULL DEFAULT 0,
    "last_update" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "xp_pkey" PRIMARY KEY ("id")
);
