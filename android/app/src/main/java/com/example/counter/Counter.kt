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
import com.example.counter.Database
import android.os.FileObserver
import kotlinx.coroutines.flow.MutableStateFlow
import java.io.File
import androidx.compose.runtime.collectAsState

@Composable
fun Counter(viewModel: ViewModel) {
    val context = LocalContext.current
    val counter = viewModel.counter.collectAsState().value

    Box(
        modifier = Modifier
            .fillMaxSize(),
        contentAlignment = Alignment.Center
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
                    text = "${counter}",
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
    }
}