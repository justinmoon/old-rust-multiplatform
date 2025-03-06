import SwiftUI

@Observable class ViewModel: FfiUpdater {
    var rust: FfiApp
    let db = Database()
    var count: String
    var router: Router
    var isSuccessScreenShown: Bool = false

    public init() {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
            .first!.absoluteString
        let rust = FfiApp(dataDir: documentsPath)
        let state = rust.getState()

        self.count = db.getCounter()
        self.router = state.router
        self.rust = rust

        self.rust.listenForUpdates(updater: self)

    }

    func update(update: Update) {
        switch update {
        case .databaseUpdate:
            self.count = db.getCounter()
            if let currentRouteName = db.getCurrentRoute() {
                if let route = routeFromString(currentRouteName) {
                    self.router.route = route
                }
            }
        }
    }

    // FIXME: This sucks. I wish there were a way to automate this ...
    private func routeFromString(_ routeName: String) -> Route? {
        switch routeName {
        case "counter": return .counter
        case "timer": return .timer
        case "home": return .home
        case "mint": return .mint
        case "mint_amount": return .mintAmount
        case "mint_confirm": return .mintConfirm
        case "melt": return .melt
        case "melt_confirm": return .meltConfirm
        case "transaction_history": return .transactionHistory
        case "success": return .success
        case "error": return .error
        default: return nil
        }
    }

    public func dispatch(event: Event) {
        self.rust.dispatch(event: event)
    }

    // This method is no longer needed since we're using Route directly
    // It remains to satisfy the compiler but shouldn't be used anymore
    func routeToAppScreen(_ route: Route) -> Route {
        return route
    }
}
