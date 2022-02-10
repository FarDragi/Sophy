-- CreateTable
CREATE TABLE "xp" (
    "id" CHAR(20) NOT NULL,
    "progress" BIGINT NOT NULL DEFAULT 0,
    "level" INTEGER NOT NULL DEFAULT 1,
    "last_update" BIGINT NOT NULL,

    CONSTRAINT "xp_pkey" PRIMARY KEY ("id")
);
