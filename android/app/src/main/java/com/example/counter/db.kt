package com.example.counter

import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import android.content.Context

class DatabaseHelper(context: Context) : SQLiteOpenHelper(context, context.filesDir.resolve("app_state.db").absolutePath, null, 1) {
    // init {
    //     val dbPath = context.filesDir.resolve("app_state.db").absolutePath
    //     // val dbFile = context.getDatabasePath(dbPath)
    //     dbFile.parentFile?.mkdirs()
    // }

    override fun onCreate(db: SQLiteDatabase) {
        // Tables are created by Rust, so no need to create them here
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        // Handle database upgrade if needed
    }

    fun getState(): String {
        // FIXME: brittle initialization code that only matters first time we run
//        try {
//            Thread.sleep(100)
//        } catch (e: InterruptedException) {
//            e.printStackTrace()
//        }
        val db = this.readableDatabase
        val cursor = db.rawQuery("SELECT state FROM app_state ORDER BY id DESC LIMIT 1", null)
        return if (cursor.moveToFirst()) {
            cursor.getString(0)
        } else {
            ""
        }.also {
            cursor.close()
        }
    }
}