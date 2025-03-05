package com.example.counter

import android.content.Context
import androidx.compose.runtime.remember
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.counter.Event
import uniffi.counter.FfiApp
import uniffi.counter.FfiUpdater
import uniffi.counter.Router
import uniffi.counter.TimerState
import uniffi.counter.Update

class ViewModel(context: Context) : ViewModel(), FfiUpdater  {
    val databaseHelper = DatabaseHelper(context);

    private val rust: FfiApp

    private var _counter: MutableStateFlow<Int>
    val counter: StateFlow<Int> get() = _counter

    private var _timer: MutableStateFlow<TimerState>
    val timer: StateFlow<TimerState> get() = _timer

    private var _router: MutableStateFlow<Router>
    val router: MutableStateFlow<Router> get() = _router

    private var _state: MutableStateFlow<String>
    val state: StateFlow<String> get() = _state

    init {
        val dataDir = context.filesDir.absolutePath
        rust = FfiApp(dataDir)
        rust.listenForUpdates(this)

        val rustState = rust.getState()
        _counter = MutableStateFlow(rustState.count)
        _timer = MutableStateFlow(rustState.timer)
        _router = MutableStateFlow(rustState.router)
        _state = MutableStateFlow(databaseHelper.getState())
    }

    override fun update(update: Update) {
        android.util.Log.d("DatabaseCheck", "update $update")
        when (update) {
            is Update.CountChanged -> {
                _counter.value = update.count
            }
            is Update.Timer -> {
                _timer.value = update.state
            }
            is Update.RouterUpdate -> {
                _router.value = update.router
            }
            is Update.DatabaseUpdate -> {
                _state.value = databaseHelper.getState();
            }
        }
    }

    fun dispatch(event: Event) {
        rust.dispatch(event)
    }
}