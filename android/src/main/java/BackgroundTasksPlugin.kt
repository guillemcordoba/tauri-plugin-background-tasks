package studio.darksoil.tauripluginbackgroundtasks

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Channel
import androidx.work.Data
import androidx.work.PeriodicWorkRequest
import androidx.work.WorkManager
import androidx.work.ExistingPeriodicWorkPolicy
import java.util.concurrent.TimeUnit

@InvokeArg
class SetupChannelArgs {
    lateinit var handler: Channel
}

@InvokeArg
class ScheduleBackgroundTaskArgs {
  lateinit var label: String
  var interval: Int = 0
}

@TauriPlugin
class BackgroundTasksPlugin(private val activity: Activity): Plugin(activity) {

    companion object {
        var channel: Channel? = null
    }

    @Command
    fun setupChannel(invoke: Invoke) {
        val args = invoke.parseArgs(SetupChannelArgs::class.java)
        BackgroundTasksPlugin.channel = args.handler
        invoke.resolve()
    }

    @Command
    fun scheduleBackgroundTask(invoke: Invoke) {
        val args = invoke.parseArgs(ScheduleBackgroundTaskArgs::class.java)

        val data = Data.Builder()
            .putString("label", args.label)
            .build()

        val work = PeriodicWorkRequest.Builder(RunnerWorker::class.java, args.interval.toLong(), TimeUnit.MINUTES)
            .setInitialDelay(args.interval.toLong(), TimeUnit.MINUTES)
            .addTag(args.label)
            .setInputData(data)
            .build()
        WorkManager.getInstance(activity).enqueueUniquePeriodicWork(args.label, ExistingPeriodicWorkPolicy.UPDATE, work)
        
        // ret.put("value", implementation.pong(args.value ?: "default value :("))
        invoke.resolve()
    }
}
