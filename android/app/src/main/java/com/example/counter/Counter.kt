import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.counter.ViewModel
import uniffi.counter.Event
import com.example.counter.DatabaseHelper // Import your DatabaseHelper

@Composable
fun Counter(viewModel: ViewModel) {
    val count by viewModel.counter.collectAsState()
    val context = LocalContext.current
    val databaseHelper = remember { DatabaseHelper(context) }
    val state = remember { mutableStateOf("") }

    // Read the state from the database
    LaunchedEffect(Unit) {
        state.value = databaseHelper.getState()
    }

    Box(
        modifier = Modifier
            .fillMaxSize(),
        contentAlignment = Alignment.Center
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.Center
            ) {
                Button(
                    onClick = { viewModel.dispatch(Event.Decrement) },
                    colors = ButtonDefaults.buttonColors(containerColor = Color.Red),
                    modifier = Modifier
                        .size(64.dp)
                ) {
                    Text("-", color = Color.White, fontSize = 40.sp)
                }

                Text(
                    text = "$count",
                    fontSize = 32.sp,
                    modifier = Modifier.padding(horizontal = 16.dp)
                )

                Button(
                    onClick = { viewModel.dispatch(Event.Increment) },
                    colors = ButtonDefaults.buttonColors(containerColor = Color.Green),
                    modifier = Modifier
                        .size(64.dp)
                ) {
                    Text("+", color = Color.White, fontSize = 32.sp)
                }
            }

            // Display the state from the database
            Text(
                text = "State: ${state.value}",
                fontSize = 24.sp,
                modifier = Modifier.padding(top = 16.dp)
            )
        }
    }
}