package com.example.counter

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.counter.Event

@Composable
fun Timer(viewModel: ViewModel) {
    val timer by viewModel.timer.collectAsState()

    Column(modifier = Modifier.padding(16.dp)) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        ) {
            Text(
                text = "${timer.elapsedSecs}",
                fontSize = 32.sp,
                modifier = Modifier.padding(horizontal = 16.dp)
            )
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        ) {
            if (timer.active) {
                Button(onClick = { viewModel.dispatch(Event.TimerPause) }) {
                    Text(text = "Stop")
                }
            } else {
                Button(onClick = { viewModel.dispatch(Event.TimerStart) }) {
                    Text(text = "Start")
                }
            }
            Button(onClick = { viewModel.dispatch(Event.TimerReset) }) {
                Text(text = "Reset")
            }
        }
    }
}