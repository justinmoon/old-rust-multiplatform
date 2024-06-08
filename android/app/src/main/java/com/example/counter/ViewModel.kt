package com.example.counter

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.counter.Event
import uniffi.counter.FfiApp
import uniffi.counter.FfiUpdater
import uniffi.counter.Update

class CounterViewModel : ViewModel(), FfiUpdater  {
    private val rust: FfiApp = FfiApp()

    private var _counter = MutableStateFlow(0)
    val counter: StateFlow<Int> = _counter

    init {
        rust.listenForUpdates(this)
    }

    override fun update(update: Update) {
        when (update) {
            is Update.CountChanged -> {
                _counter.value = update.count
            }
        }
    }

    fun dispatch(event: Event) {
        rust.dispatch(event)
    }
}