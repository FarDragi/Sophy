-- CreateTable
CREATE TABLE "xp" (
    "id" UUID NOT NULL,
    "level" INTEGER NOT NULL DEFAULT 1,
    "progress" BIGINT NOT NULL DEFAULT 0,
    "user_id" TEXT NOT NULL,

    CONSTRAINT "xp_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "xp_user_id_key" ON "xp"("user_id");

-- AddForeignKey
ALTER TABLE "xp" ADD CONSTRAINT "xp_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
