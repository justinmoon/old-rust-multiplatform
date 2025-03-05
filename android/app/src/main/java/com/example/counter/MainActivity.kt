package com.example.counter

import Counter
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import com.example.counter.ui.theme.CounterTheme
import uniffi.counter.Event
import uniffi.counter.Route

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            CounterTheme {
                App()
            }
        }
    }
}

@Composable
fun App() {
    val context = LocalContext.current
    val viewModel = ViewModel(context);
    val router by viewModel.router.collectAsState()

    Column(modifier = Modifier.padding(16.dp)) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        ) {
            Button(onClick = { viewModel.dispatch(Event.SetRoute(Route.COUNTER)) }) {
                Text(text = "Counter")
            }
            Button(onClick = { viewModel.dispatch(Event.SetRoute(Route.TIMER)) }) {
                Text(text = "Timer")
            }

        }
        if (router.route == Route.COUNTER) {
            Counter(viewModel)
        } else {
            Timer(viewModel)
        }
    }
}

