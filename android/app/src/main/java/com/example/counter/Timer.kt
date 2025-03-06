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
    Text(
        text = "Timer",
        fontSize = 32.sp,
        modifier = Modifier.padding(horizontal = 16.dp)
    )
}