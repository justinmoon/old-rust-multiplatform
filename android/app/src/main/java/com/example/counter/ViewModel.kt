package com.example.counter

import android.content.Context
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
    private val rust: FfiApp

    private var _counter: MutableStateFlow<Int>
    val counter: StateFlow<Int> get() = _counter

    private var _timer: MutableStateFlow<TimerState>
    val timer: StateFlow<TimerState> get() = _timer

    private var _router: MutableStateFlow<Router>
    val router: MutableStateFlow<Router> get() = _router

    init {
        val dataDir = context.filesDir.absolutePath
        rust = FfiApp(dataDir)
        rust.listenForUpdates(this)

        val state = rust.getState()
        _counter = MutableStateFlow(state.count)
        _timer = MutableStateFlow(state.timer)
        _router = MutableStateFlow(state.router)
    }

    override fun update(update: Update) {
        println("update $update")
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
        }
        println(_counter.value)
        println(_timer.value)
        println(_router.value)
        println()

    }

    fun dispatch(event: Event) {
        rust.dispatch(event)
    }
}