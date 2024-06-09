package com.example.counter

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.counter.Event
import uniffi.counter.FfiApp
import uniffi.counter.FfiUpdater
import uniffi.counter.TimerState
import uniffi.counter.Update

class ViewModel : ViewModel(), FfiUpdater  {
    private val rust: FfiApp = FfiApp()

    private var _counter: MutableStateFlow<Int>
    val counter: StateFlow<Int> get() = _counter

    private var _timer: MutableStateFlow<TimerState>
    val timer: StateFlow<TimerState> get() = _timer

    init {
        rust.listenForUpdates(this)

        val state = rust.getState()
        _counter = MutableStateFlow(state.count)
        _timer = MutableStateFlow(state.timer)
    }

    override fun update(update: Update) {
        when (update) {
            is Update.CountChanged -> {
                _counter.value = update.count
            }
            is Update.Timer -> {
                println("timer" + update.state)
                _timer.value = update.state
            }
        }
    }

    fun dispatch(event: Event) {
        rust.dispatch(event)
    }
}