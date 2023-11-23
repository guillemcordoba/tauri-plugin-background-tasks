package studio.darksoil.tauripluginbackgroundtasks

import android.content.Context
import android.util.Log
import androidx.work.Data
import androidx.work.Worker
import androidx.work.WorkerParameters
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.runBlocking
import app.tauri.plugin.JSObject

class RunnerWorker(context: Context, workerParams: WorkerParameters) : Worker(context, workerParams) {
    override fun doWork(): Result {
        try {
            val label = this.inputData.getString("label") ?: ""

            if (label.isEmpty()) {
                throw Exception("label is empty")
            }

            runBlocking {
                val event = JSObject()
                event.put("label", label)
                BackgroundTasksPlugin.channel?.send(event)
              //  val impl = BackgroundRunner.getInstance(this@RunnerWorker.applicationContext)
              //  impl.execute(this@RunnerWorker.applicationContext, config, JSObject())
            }

            return Result.success()
        } catch (ex: Exception) {
            val label = this.inputData.getString("label") ?: ""
            Log.e("[RUNNER WORKER for $label]", ex.toString())
            val data = Data.Builder()
                .putString("error", ex.toString())
                .build()

            return Result.failure(data)
        }
    }
}