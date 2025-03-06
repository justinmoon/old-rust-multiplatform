//
//  CounterApp.swift
//  Counter
//
//  Created by Justin  on 6/4/24.
//

import Counter
import SwiftUI

@main
struct CounterApp: App {
    @State var rust: ViewModel

    public init() {
        self.rust = ViewModel()
    }

    var body: some Scene {
        WindowGroup {
            AppNavigation(rust: rust)
        }
    }
}

struct MainContentView: View {
    @State var rust: ViewModel
    @State private var tabSelection: Int = 0
    @State private var showSuccess: Bool = false
    @State private var showError: Bool = false
    @State private var successMessage: String = ""
    @State private var errorMessage: String = ""

    var body: some View {
        NavigationStack {
            TabView(selection: $tabSelection) {
                HomeView(rust: rust)
                    .tabItem {
                        Label("Home", systemImage: "house")
                    }
                    .tag(0)

                TransactionHistoryView(rust: rust)
                    .tabItem {
                        Label("History", systemImage: "clock")
                    }
                    .tag(1)
            }
            .onChange(of: tabSelection) { oldValue, newValue in
                if newValue == 1 {
                    rust.dispatch(event: .pushRoute(route: .transactionHistory))
                } else {
                    rust.dispatch(event: .resetRouter)
                }
            }
            .fullScreenCover(isPresented: $showSuccess) {
                SuccessView(
                    rust: self.rust, message: successMessage,
                    onDismiss: {
                        showSuccess = false
                    })
            }
            .fullScreenCover(isPresented: $showError) {
                ErrorView(
                    rust: self.rust, error: errorMessage,
                    onRetry: {
                        // Handle retry logic
                        showError = false
                    },
                    onQuit: {
                        showError = false
                    })
            }
        }
        //        .environmentObject(rust)
        .onChange(of: rust.currentRoute) { oldValue, newValue in
            if let unwrappedRoute = newValue {
                handleRouteChange(unwrappedRoute)
            }
        }
    }

    private func handleRouteChange(_ route: Route) {
        switch route {
        case .home:
            tabSelection = 0
        case .transactionHistory:
            tabSelection = 1
        case .success:
            successMessage = "Transaction completed successfully!"
            showSuccess = true
        case .error:
            errorMessage = "Transaction failed. Please try again."
            showError = true
        default:
            // Handle other routes if needed
            break
        }
    }
}
