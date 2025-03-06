package com.example.counter

import android.content.Context
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.counter.Event
import uniffi.counter.FfiApp
import uniffi.counter.FfiUpdater
import uniffi.counter.Router
import uniffi.counter.Update

class ViewModel(context: Context) : ViewModel(), FfiUpdater  {
    val db = Database(context);

    private val rust: FfiApp

    private var _counter: MutableStateFlow<String>
    val counter: StateFlow<String> get() = _counter

    private var _router: MutableStateFlow<Router>
    val router: MutableStateFlow<Router> get() = _router

    init {
        val dataDir = context.filesDir.absolutePath
        rust = FfiApp(dataDir)
        rust.listenForUpdates(this)

        val rustState = rust.getState()
        _router = MutableStateFlow(rustState.router)
        _counter = MutableStateFlow(db.getCounter())
    }

    override fun update(update: Update) {
        android.util.Log.d("DatabaseCheck", "update $update")
        when (update) {
            is Update.RouterUpdate -> {
                _router.value = update.router
            }
            is Update.DatabaseUpdate -> {
                _counter.value = db.getCounter();
            }
        }
    }

    fun dispatch(event: Event) {
        rust.dispatch(event)
    }
}