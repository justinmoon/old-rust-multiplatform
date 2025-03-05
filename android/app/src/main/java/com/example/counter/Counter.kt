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
import com.example.counter.DatabaseHelper
import android.os.FileObserver
import kotlinx.coroutines.flow.MutableStateFlow
import java.io.File

@Composable
fun Counter(viewModel: ViewModel) {
    val context = LocalContext.current
    val databaseHelper = remember { DatabaseHelper(context) }
    val state = remember { mutableStateOf("") }

    // Manual reloading logic using FileObserver
    val databaseFile = File(context.filesDir, "app_state.db")
    val lastModified = remember { MutableStateFlow(databaseFile.lastModified()) }

    DisposableEffect(Unit) {
        val observer = object : FileObserver(databaseFile.path, MODIFY) {
            override fun onEvent(event: Int, path: String?) {
                if (event == MODIFY) {
                    state.value = databaseHelper.getState()
                }
            }
        }
        observer.startWatching()

        onDispose {
            observer.stopWatching()
        }
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
                    text = "${viewModel.counter.collectAsState().value}",
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
                text = "Counter: ${state.value}",
                fontSize = 24.sp,
                modifier = Modifier.padding(top = 16.dp)
            )
        }
    }
}