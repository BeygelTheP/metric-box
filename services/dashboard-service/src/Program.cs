using Serilog;

Log.Logger = new LoggerConfiguration()
    .WriteTo.Console()
    .CreateLogger();

try
{
    Log.Information("🚀 Starting MetricBox Dashboard Service Service");

    var builder = WebApplication.CreateBuilder(args);
    
    builder.Host.UseSerilog();
    builder.Services.AddControllers();
    builder.Services.AddEndpointsApiExplorer();
    builder.Services.AddSwaggerGen();

    var app = builder.Build();

    if (app.Environment.IsDevelopment())
    {
        app.UseSwagger();
        app.UseSwaggerUI();
    }

    app.UseHttpsRedirection();
    app.UseAuthorization();
    app.MapControllers();

    app.MapGet("/health", () => Results.Ok(new { status = "healthy", service = "metricbox-dashboard-service" }));

    Log.Information("📦 MetricBox Dashboard Service Service is ready!");
    
    app.Run();
}
catch (Exception ex)
{
    Log.Fatal(ex, "❌ MetricBox Dashboard Service Service terminated unexpectedly");
}
finally
{
    Log.CloseAndFlush();
}
