package com.example.counter

import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import android.content.Context

// FIXME: would be better to pass the path in directly than calculate it here ...
class DatabaseHelper(context: Context) : SQLiteOpenHelper(context, context.filesDir.resolve("app_state.db").absolutePath, null, 1) {
    override fun onCreate(db: SQLiteDatabase) {
        // Tables are created by Rust, so no need to create them here
    }

    override fun onUpgrade(db: SQLiteDatabase, oldVersion: Int, newVersion: Int) {
        // Handle database upgrade if needed
    }

    fun getState(): String {
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