-- CreateTable
CREATE TABLE "Xp" (
    "id" UUID NOT NULL,
    "level" INTEGER NOT NULL DEFAULT 1,
    "progress" BIGINT NOT NULL DEFAULT 0,
    "userId" TEXT,

    CONSTRAINT "Xp_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Xp_userId_key" ON "Xp"("userId");

-- AddForeignKey
ALTER TABLE "Xp" ADD CONSTRAINT "Xp_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE SET NULL ON UPDATE CASCADE;
