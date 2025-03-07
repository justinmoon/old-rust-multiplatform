import Foundation
import SQLite3

class Database {
    private var db: OpaquePointer?
    private let dbName = "app_state.db"

    init() {
        openDatabase()
    }

    private func openDatabase() {
        let fileManager = FileManager.default
        let documentsUrl = fileManager.urls(for: .documentDirectory, in: .userDomainMask).first!
        let databaseUrl = documentsUrl.appendingPathComponent(dbName)

        if sqlite3_open(databaseUrl.path, &db) != SQLITE_OK {
            print("Unable to open database.")
        }
    }

    deinit {
        sqlite3_close(db)
    }

    func getCurrentRoute() -> String? {
        let query = "SELECT route_name FROM navigation_stack ORDER BY id DESC LIMIT 1"
        var queryStatement: OpaquePointer?
        var routeName: String? = nil

        if sqlite3_prepare_v2(db, query, -1, &queryStatement, nil) == SQLITE_OK {
            if sqlite3_step(queryStatement) == SQLITE_ROW {
                if let queryResultCol1 = sqlite3_column_text(queryStatement, 0) {
                    routeName = String(cString: queryResultCol1)
                }
            }
        }
        sqlite3_finalize(queryStatement)
        return routeName
    }
}
