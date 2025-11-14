"use client"

import { X, MapPin, Briefcase, Clock, Building2, DollarSign, Users, Bookmark, Target, TrendingUp, CheckCircle2, AlertCircle, ExternalLink, Linkedin, Globe } from "lucide-react"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { ScrollArea } from "@/components/ui/scroll-area"
import { Progress } from "@/components/ui/progress"
import { Separator } from "@/components/ui/separator"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { useRouter } from "next/navigation"

interface JobDetailsModalProps {
  open: boolean
  onClose: () => void
  job: {
    id: string
    title: string
    company: string
    location: string
    type: string
    salary?: string
    experience: string
    skills: string[]
    postedDate: string
    description: string
    responsibilities: string[]
    requirements: string[]
    benefits?: string[]
    matchScore?: number
    matchExplanation?: string
    strengths?: string[]
    improvementAreas?: string[]
    experienceAlignment?: number
    trackAlignment?: number
    skillOverlap?: number
    platformLinks?: {
      linkedin: string
      bdjobs: string
      glassdoor: string
      indeed: string
    }
  } | null
}

export default function JobDetailsModal({ open, onClose, job }: JobDetailsModalProps) {
  const router = useRouter()
  
  if (!job) return null

  const handleSaveJob = () => {
    // Placeholder for save functionality
    alert(`Job "${job.title}" saved!`)
  }

  const handleAnalyzeSkills = () => {
    onClose()
    // Navigate to skill gap page with the job title pre-filled
    router.push(`/skill-gap?role=${encodeURIComponent(job.title)}`)
  }

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="glass-effect border-white/10 max-w-3xl max-h-[90vh] p-0">
        <ScrollArea className="max-h-[90vh]">
          <div className="p-6 space-y-6">
            {/* Header */}
            <DialogHeader>
              <div className="flex items-start justify-between gap-4">
                <div className="space-y-3 flex-1">
                  <DialogTitle className="text-2xl font-bold text-gradient">
                    {job.title}
                  </DialogTitle>
                  <div className="flex items-center space-x-2">
                    <Building2 className="w-5 h-5 text-blue-400" />
                    <span className="text-lg font-medium text-foreground">{job.company}</span>
                  </div>
                </div>
                <Badge
                  variant="secondary"
                  className="glass-effect text-sm font-medium px-4 py-2 border border-blue-500/30"
                >
                  {job.type}
                </Badge>
              </div>
            </DialogHeader>

            {/* Key Info */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div className="flex items-center space-x-3 glass-effect rounded-lg p-3">
                <MapPin className="w-5 h-5 text-blue-400 shrink-0" />
                <div>
                  <p className="text-xs text-muted-foreground">Location</p>
                  <p className="text-sm font-medium">{job.location}</p>
                </div>
              </div>

              {job.salary && (
                <div className="flex items-center space-x-3 glass-effect rounded-lg p-3">
                  <DollarSign className="w-5 h-5 text-green-400 shrink-0" />
                  <div>
                    <p className="text-xs text-muted-foreground">Salary</p>
                    <p className="text-sm font-medium">{job.salary}</p>
                  </div>
                </div>
              )}

              <div className="flex items-center space-x-3 glass-effect rounded-lg p-3">
                <Users className="w-5 h-5 text-purple-400 shrink-0" />
                <div>
                  <p className="text-xs text-muted-foreground">Experience</p>
                  <p className="text-sm font-medium">{job.experience}</p>
                </div>
              </div>

              <div className="flex items-center space-x-3 glass-effect rounded-lg p-3">
                <Clock className="w-5 h-5 text-orange-400 shrink-0" />
                <div>
                  <p className="text-xs text-muted-foreground">Posted</p>
                  <p className="text-sm font-medium">{job.postedDate}</p>
                </div>
              </div>
            </div>

            {/* Skills */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold text-foreground">Required Skills</h3>
              <div className="flex flex-wrap gap-2">
                {job.skills.map((skill, index) => (
                  <Badge
                    key={index}
                    variant="outline"
                    className="text-sm px-3 py-1 border-blue-400/30 hover:border-blue-400 transition-colors"
                  >
                    {skill}
                  </Badge>
                ))}
              </div>
            </div>

            {/* Description */}
            <div className="space-y-3">
              <h3 className="text-lg font-semibold text-foreground">Job Description</h3>
              <p className="text-sm text-muted-foreground leading-relaxed">{job.description}</p>
            </div>

            {/* Responsibilities */}
            {job.responsibilities && job.responsibilities.length > 0 && (
              <div className="space-y-3">
                <h3 className="text-lg font-semibold text-foreground">Responsibilities</h3>
                <ul className="space-y-2">
                  {job.responsibilities.map((item, index) => (
                    <li key={index} className="flex items-start space-x-2 text-sm text-muted-foreground">
                      <span className="text-blue-400 mt-1">•</span>
                      <span>{item}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {/* Requirements */}
            {job.requirements && job.requirements.length > 0 && (
              <div className="space-y-3">
                <h3 className="text-lg font-semibold text-foreground">Requirements</h3>
                <ul className="space-y-2">
                  {job.requirements.map((item, index) => (
                    <li key={index} className="flex items-start space-x-2 text-sm text-muted-foreground">
                      <span className="text-purple-400 mt-1">•</span>
                      <span>{item}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {/* Benefits */}
            {job.benefits && job.benefits.length > 0 && (
              <div className="space-y-3">
                <h3 className="text-lg font-semibold text-foreground">Benefits</h3>
                <ul className="space-y-2">
                  {job.benefits.map((item, index) => (
                    <li key={index} className="flex items-start space-x-2 text-sm text-muted-foreground">
                      <span className="text-green-400 mt-1">•</span>
                      <span>{item}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {/* Match Score Card */}
            {job.matchScore != null && !isNaN(job.matchScore) && (
              <>
                <Separator className="my-6" />
                <Card className="border-green-200/50 dark:border-green-500/30 bg-gradient-to-br from-green-50/50 to-emerald-50/50 dark:from-green-950/30 dark:to-emerald-950/30">
                  <CardHeader>
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <TrendingUp className="w-5 h-5 text-green-500" />
                        <CardTitle className="text-xl">Match Score</CardTitle>
                      </div>
                      <Badge className="text-lg font-bold px-4 py-1 bg-green-500/20 text-green-700 dark:text-green-300 border-green-500/50">
                        {job.matchScore.toFixed(0)}%
                      </Badge>
                    </div>
                    {job.matchExplanation && (
                      <CardDescription className="text-sm text-muted-foreground mt-2">
                        {job.matchExplanation}
                      </CardDescription>
                    )}
                  </CardHeader>
                  <CardContent className="space-y-4">
                    {/* Match Breakdown */}
                    <div className="space-y-3">
                      {job.skillOverlap !== undefined && (
                        <div className="space-y-1">
                          <div className="flex items-center justify-between text-sm">
                            <span className="text-muted-foreground">Skill Overlap</span>
                            <span className="font-medium">{job.skillOverlap.toFixed(0)}%</span>
                          </div>
                          <Progress value={job.skillOverlap} className="h-2" />
                        </div>
                      )}
                      {job.experienceAlignment !== undefined && (
                        <div className="space-y-1">
                          <div className="flex items-center justify-between text-sm">
                            <span className="text-muted-foreground">Experience Alignment</span>
                            <span className="font-medium">{job.experienceAlignment.toFixed(0)}%</span>
                          </div>
                          <Progress value={job.experienceAlignment} className="h-2" />
                        </div>
                      )}
                      {job.trackAlignment !== undefined && (
                        <div className="space-y-1">
                          <div className="flex items-center justify-between text-sm">
                            <span className="text-muted-foreground">Career Track Alignment</span>
                            <span className="font-medium">{job.trackAlignment.toFixed(0)}%</span>
                          </div>
                          <Progress value={job.trackAlignment} className="h-2" />
                        </div>
                      )}
                    </div>

                    {/* Strengths */}
                    {job.strengths && job.strengths.length > 0 && (
                      <div className="space-y-2">
                        <h4 className="text-sm font-semibold text-foreground flex items-center gap-2">
                          <CheckCircle2 className="w-4 h-4 text-green-500" />
                          Key Strengths
                        </h4>
                        <ul className="space-y-1.5">
                          {job.strengths.map((strength, index) => (
                            <li key={index} className="flex items-start space-x-2 text-sm text-muted-foreground">
                              <span className="text-green-400 mt-1">✓</span>
                              <span>{strength}</span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    )}

                    {/* Improvement Areas */}
                    {job.improvementAreas && job.improvementAreas.length > 0 && (
                      <div className="space-y-2">
                        <h4 className="text-sm font-semibold text-foreground flex items-center gap-2">
                          <AlertCircle className="w-4 h-4 text-orange-500" />
                          Areas for Improvement
                        </h4>
                        <ul className="space-y-1.5">
                          {job.improvementAreas.map((area, index) => (
                            <li key={index} className="flex items-start space-x-2 text-sm text-muted-foreground">
                              <span className="text-orange-400 mt-1">•</span>
                              <span>{area}</span>
                            </li>
                          ))}
                        </ul>
                      </div>
                    )}
                  </CardContent>
                </Card>
              </>
            )}

            {/* Platform Links */}
            {job.platformLinks && (
              <>
                <Separator className="my-6" />
                <Card className="border-blue-200/50 dark:border-blue-500/30 bg-gradient-to-br from-blue-50/50 to-indigo-50/50 dark:from-blue-950/30 dark:to-indigo-950/30">
                  <CardHeader>
                    <CardTitle className="text-lg flex items-center gap-2">
                      <Globe className="w-5 h-5 text-blue-500" />
                      Apply on Job Platforms
                    </CardTitle>
                    <CardDescription>
                      Find this job on popular job search platforms
                    </CardDescription>
                  </CardHeader>
                  <CardContent>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                      <Button
                        variant="outline"
                        className="justify-start border-blue-200/50 dark:border-blue-500/30 hover:bg-blue-50 dark:hover:bg-blue-950/50"
                        onClick={() => window.open(job.platformLinks!.linkedin, '_blank')}
                      >
                        <Linkedin className="w-4 h-4 mr-2 text-blue-600" />
                        LinkedIn
                        <ExternalLink className="w-3 h-3 ml-auto" />
                      </Button>
                      <Button
                        variant="outline"
                        className="justify-start border-blue-200/50 dark:border-blue-500/30 hover:bg-blue-50 dark:hover:bg-blue-950/50"
                        onClick={() => window.open(job.platformLinks!.bdjobs, '_blank')}
                      >
                        <Globe className="w-4 h-4 mr-2 text-green-600" />
                        BDjobs
                        <ExternalLink className="w-3 h-3 ml-auto" />
                      </Button>
                      <Button
                        variant="outline"
                        className="justify-start border-blue-200/50 dark:border-blue-500/30 hover:bg-blue-50 dark:hover:bg-blue-950/50"
                        onClick={() => window.open(job.platformLinks!.glassdoor, '_blank')}
                      >
                        <Globe className="w-4 h-4 mr-2 text-purple-600" />
                        Glassdoor
                        <ExternalLink className="w-3 h-3 ml-auto" />
                      </Button>
                      <Button
                        variant="outline"
                        className="justify-start border-blue-200/50 dark:border-blue-500/30 hover:bg-blue-50 dark:hover:bg-blue-950/50"
                        onClick={() => window.open(job.platformLinks!.indeed, '_blank')}
                      >
                        <Globe className="w-4 h-4 mr-2 text-indigo-600" />
                        Indeed
                        <ExternalLink className="w-3 h-3 ml-auto" />
                      </Button>
                    </div>
                  </CardContent>
                </Card>
              </>
            )}

            {/* Action Buttons */}
            <div className="flex flex-col sm:flex-row gap-3 pt-4">
              <Button
                onClick={handleAnalyzeSkills}
                className="flex-1 rounded-lg bg-gradient-to-r from-purple-500 to-pink-600 hover:from-purple-600 hover:to-pink-700 text-white font-medium shadow-lg hover:shadow-purple-500/50 transition-all duration-300"
              >
                <Target className="w-4 h-4 mr-2" />
                Analyze Skills
              </Button>
              <Button
                onClick={handleSaveJob}
                className="flex-1 rounded-lg bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white font-medium shadow-lg hover:shadow-blue-500/50 transition-all duration-300"
              >
                <Bookmark className="w-4 h-4 mr-2" />
                Save Job
              </Button>
              <Button
                variant="outline"
                onClick={onClose}
                className="flex-1 rounded-lg border-white/20 hover:bg-white/5"
              >
                Close
              </Button>
            </div>
          </div>
        </ScrollArea>
      </DialogContent>
    </Dialog>
  )
}